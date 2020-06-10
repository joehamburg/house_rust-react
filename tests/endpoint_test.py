import requests
BASEURL = "http://localhost:8000/homestatus"


create_user = {
    'id': 69,
    'description': 'dishwasher',
    'available': True
}

x = requests.post(BASEURL, json = create_user)