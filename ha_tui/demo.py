from requests import get

url = "http://10.40.2.101:8123/api/states"
headers = {
    "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiIyMjVkN2MwOGUwZjg0N2M5YmU5N2NlZDIwMGNiY2NmZiIsImlhdCI6MTc4MDkxMjU4MCwiZXhwIjoyMDk2MjcyNTgwfQ.gFpjPLHLu3fGk8IMC6FalquItuwTAhlDDaHDpM1DPmQ",
    "content-type": "application/json",
}

response = get(url, headers=headers)
print(response.text)
