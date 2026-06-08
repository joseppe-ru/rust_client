import logging
import aiohttp
from homeassistant.components.button import ButtonEntity
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback

from .const import DOMAIN, CONF_TUI_IP, CONF_TUI_PORT

_LOGGER = logging.getLogger(__name__)

async def async_setup_entry(
    hass: HomeAssistant, entry: ConfigEntry, async_add_entities: AddEntitiesCallback
) -> None:
    """Setzt die Buttons basierend auf dem Config Entry auf."""
    config_data = hass.data[DOMAIN][entry.entry_id]
    tui_ip = config_data[CONF_TUI_IP]
    tui_port = config_data[CONF_TUI_PORT]

    async_add_entities([RustTuiTriggerButton(tui_ip, tui_port)])

class RustTuiTriggerButton(ButtonEntity):
    """Button, der ein Signal an die Rust TUI sendet."""

    def __init__(self, ip: str, port: int):
        self._ip = ip
        self._port = port
        self._attr_name = "Signal an Rust TUI"
        self._attr_unique_id = f"rust_tui_btn_{ip}_{port}"
        self._attr_icon = "mdi:terminal"

    async def async_press(self) -> None:
        """Button wurde in HA gedrückt."""
        url = f"http://{self._ip}:{self._port}/ha-event"
        _LOGGER.info(f"Sende Trigger an Rust TUI unter: {url}")

        async with aiohttp.ClientSession() as session:
            try:
                # Wir senden ein einfaches JSON mit, was gedrückt wurde
                payload = {"event": "button_pressed", "entity_id": self.entity_id}
                async with session.post(url, json=payload, timeout=5) as response:
                    if response.status != 200:
                        _LOGGER.warning(f"TUI antwortete mit Status {response.status}")
            except Exception as e:
                _LOGGER.error(f"Fehler beim Verbinden zur TUI: {e}")
