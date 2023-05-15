import requests
import time
import json

def pretty_print(obj):
    parsed = json.loads(obj)
    print(json.dumps(parsed, indent=4, sort_keys=True))

if __name__ == '__main__':
    while True:
        requests.post('http://localhost:8080/mobile_network/update_user_positions')
        edcs = requests.get('http://localhost:8080/network/edge_data_centers').json()
        for edc in edcs:
            applications = requests.get(f"http://localhost:8080/network/edge_data_centers/{edc['id']}/applications").json()
            for application in applications:
                print(edc['id'], application['id'])
        # time.sleep(0.1)
