import requests
import time
import json

def pretty_print(obj):
    parsed = json.loads(obj)
    print(json.dumps(parsed, indent=4, sort_keys=True))

if __name__ == '__main__':
    time_to_run = time.time() + 300
    result = []
    while time.time() < time_to_run:
        timestamp = time.time()
        requests.post('http://localhost:8080/mobile_network/update_user_positions')
        edcs = requests.get('http://localhost:8080/network/edge_data_centers').json()
        for edc in edcs:
            applications = requests.get(f"http://localhost:8080/network/edge_data_centers/{edc['id']}/applications").json()
            for application in applications:
                print(time_to_run - timestamp, edc['id'], application['id'])
                result.append({'timestamp': timestamp, 'edc_id': edc['id'], 'app_id': application['id']})

    with open('application_locations.json', 'w') as f:
        json.dump(result, f)
