import requests
import time
import json

def pretty_print(obj):
    parsed = json.loads(obj)
    print(json.dumps(parsed, indent=4, sort_keys=True))

if __name__ == '__main__':
    while True:
        requests.post('http://localhost:8080/mobile_network/update_user_positions')
        connected_users = requests.get('http://localhost:8080/mobile_network/connected_users').text
        pretty_print(connected_users)
        time.sleep(0.2)
