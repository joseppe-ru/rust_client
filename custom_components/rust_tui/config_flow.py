import voluptuous as vol
from homeassistant import config_entries
from homeassistant.core import callback
import homeassistant.helpers.config_validation as cv

from .const import DOMAIN, CONF_TUI_IP, CONF_TUI_PORT

class RustTuiConfigFlow(config_entries.ConfigFlow, domain=DOMAIN):
    """Verwaltet den Config Flow für die Rust TUI."""

    VERSION = 1

    async def async_step_user(self, user_input=None):
        """Wird aufgerufen, wenn der User die Integration über die UI hinzufügt."""
        errors = {}

        if user_input is not None:
            # Hier könnte man prüfen, ob die IP erreichbar ist (Ping oder Test-Request)
            # Wenn alles okay ist, Eintrag erstellen:
            return self.async_create_entry(
                title=f"Rust TUI ({user_input[CONF_TUI_IP]})", 
                data=user_input
            )

        # Das Schema definiert die Eingabemaske in der UI
        DATA_SCHEMA = vol.Schema(
            {
                vol.Required(CONF_TUI_IP, default="192.168.1.X"): cv.string,
                vol.Required(CONF_TUI_PORT, default=8080): cv.port,
            }
        )

        return self.async_show_form(
            step_id="user", data_schema=DATA_SCHEMA, errors=errors
        )
