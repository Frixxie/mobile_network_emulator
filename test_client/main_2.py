import requests
import time

if __name__ == '__main__':

    body = {
            'notify_endpoint': 'http://192.168.1.7:8567/pdn_connection_event',
            'kind': 'PdnConnectionEvent',
            'user_ids': [i for i in range(4)]
    }

    res = requests.post('http://localhost:8080/mobile_network_exposure/subscribers', json=body)
    print(res.status_code)
    print(res.text)

    body = {
            'notify_endpoint': 'http://192.168.1.7:8567/location_reporting',
            'kind': 'LocationReporting',
            'user_ids': [i for i in range(4)]
    }

    res = requests.post('http://localhost:8080/mobile_network_exposure/subscribers', json=body)
    print(res.status_code)
    print(res.text)


    while True:
        res = requests.post('http://localhost:8080/mobile_network_exposure/events/publish')
        print(res.status_code)
        print(res.text)

        time.sleep(0.5)
