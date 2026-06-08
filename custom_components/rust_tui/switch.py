import logging
import aiohttp
from typing import Any
from homeassistant.components.switch import SwitchEntity
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback

from .const import DOMAIN, CONF_TUI_IP, CONF_TUI_PORT

_LOGGER = logging.getLogger(__name__)

async def async_setup_entry(
    hass: HomeAssistant, entry: ConfigEntry, async_add_entities: AddEntitiesCallback
) -> None:
    """Setzt die Schalter auf."""
    config = hass.data[DOMAIN][entry.entry_id]
    async_add_entities([RustTuiMuteSwitch(config[CONF_TUI_IP], config[CONF_TUI_PORT])])

class RustTuiMuteSwitch(SwitchEntity):
    """Ein Schalter, um die Rust TUI stummzuschalten."""

    def __init__(self, ip: str, port: int):
        self._ip = ip
        self._port = port
        self._attr_name = "Mute"
        self._attr_unique_id = f"rust_tui_mute_{ip}_{port}"
        self._attr_icon = "mdi:volume-off"
        # Standardmäßig ist der Schalter aus
        self._attr_is_on = False 

    async def _send_command(self, action: str):
        """Hilfsfunktion für den HTTP Request an Rust."""
        url = f"http://{self._ip}:{self._port}/ha-event"
        payload = {"event": action, "entity_id": self.entity_id}
        
        async with aiohttp.ClientSession() as session:
            try:
                async with session.post(url, json=payload, timeout=5) as response:
                    return response.status == 200
            except Exception as e:
                _LOGGER.error(f"Fehler beim Schalten: {e}")
                return False

    async def async_turn_on(self, **kwargs: Any) -> None:
        """Schaltet den Mute-Modus AN."""
        success = await self._send_command("mute_on")
        if success:
            self._attr_is_on = True
            # Update das HA Dashboard
            self.async_write_ha_state()

    async def async_turn_off(self, **kwargs: Any) -> None:
        """Schaltet den Mute-Modus AUS."""
        success = await self._send_command("mute_off")
        if success:
            self._attr_is_on = False
            self.async_write_ha_state()
