import logging
from homeassistant.components.sensor import SensorEntity, SensorDeviceClass
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback
from datetime import datetime

from .const import DOMAIN, CONF_TUI_IP

_LOGGER = logging.getLogger(__name__)

async def async_setup_entry(
    hass: HomeAssistant, entry: ConfigEntry, async_add_entities: AddEntitiesCallback
) -> None:
    """Setzt die Sensoren auf."""
    ip = hass.data[DOMAIN][entry.entry_id][CONF_TUI_IP]
    async_add_entities([RustTuiLastOnlineSensor(ip)])

class RustTuiLastOnlineSensor(SensorEntity):
    """Zeigt an, wann die TUI zuletzt ein Lebenszeichen gesendet hat."""

    def __init__(self, ip: str):
        self._attr_name = "Zuletzt Online"
        self._attr_unique_id = f"rust_tui_last_online_{ip}"
        self._attr_icon = "mdi:clock-check-outline"
        # device_class timestamp sorgt dafür, dass HA die Zeit schön als "vor 5 Minuten" anzeigt
        self._attr_device_class = SensorDeviceClass.TIMESTAMP 
        
        # Der eigentliche Wert
        self._attr_native_value = None 

    def update_last_online(self, new_time: datetime):
        """
        Diese Methode kannst du von einem anderen Ort aufrufen 
        (z.B. einem Webhook-Listener in Python), wenn die TUI "Hallo" sagt.
        """
        self._attr_native_value = new_time
        # WICHTIG: Teilt HA mit, dass sich der Wert geändert hat und das UI aktualisiert werden muss
        self.async_write_ha_state()
