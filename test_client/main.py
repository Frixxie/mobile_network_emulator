import requests
import time

if __name__ == '__main__':

    form = {"username": "admin@my-email.com", "password": "pass"}
    session = requests.Session()
    res = session.post(
        'http://localhost:8888/api/v1/login/access-token', data=form).json()

    print(res)

    token = res['access_token']
    type = res['token_type']

    header = {"Authorization": f"{type} {token}"}

    for id in ["10001@domain.com", "10002@domain.com", "10003@domain.com"]:
        body = {
            "externalId": id,
            "notificationDestination": "http://172.18.0.1:8789/",
            "monitoringType": "LOCATION_REPORTING",
            "maximumNumberOfReports": 20,
            "monitorExpireTime": "2023-01-27T17:00:09.012Z",
            "maximumDetectionTime": 1,
            "reachabilityType": "DATA"
        }

        res = session.post(
            'http://localhost:8888/nef/api/v1/3gpp-monitoring-event/v1/testApp/subscriptions', json=body, headers=header)

        print(res.status_code)
        print(res.text)

    while True:
        res = session.get(
            'http://localhost:8888/nef/api/v1/3gpp-monitoring-event/v1/testApp/subscriptions', headers=header)
        print(res.status_code)
        print(res.text)

        time.sleep(5)
