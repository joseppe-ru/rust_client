from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant

from .const import DOMAIN

async def async_setup_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    """Setzt die Integration aus einem Config Entry (UI-Eintrag) auf."""
    hass.data.setdefault(DOMAIN, {})
    hass.data[DOMAIN][entry.entry_id] = entry.data

    # Leite das Setup an die button.py weiter
    await hass.config_entries.async_forward_entry_setups(entry, ["button"])
    return True

async def async_unload_entry(hass: HomeAssistant, entry: ConfigEntry) -> bool:
    """Wird aufgerufen, wenn die Integration gelöscht wird."""
    unload_ok = await hass.config_entries.async_unload_platforms(entry, ["button"])
    if unload_ok:
        hass.data[DOMAIN].pop(entry.entry_id)
    return unload_ok
