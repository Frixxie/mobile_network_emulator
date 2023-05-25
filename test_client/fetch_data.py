import matplotlib.pyplot as plt 
from pymongo import MongoClient
import datetime
import numpy as np
import time
import json

if __name__ == '__main__':
    client = MongoClient()
    db = client['mn_system']
    collection = db['NetworkLog']

    time_to_run = time.time() + 300

    result = []
    while time.time() < time_to_run:
        timestamp = time.time()
        data = list(collection.find({'timestamp': {'$gt': time.time() - 15}}))
        avg = []
        for a in range(8):
            for i in range(128):
                xs = [datetime.datetime.fromtimestamp(d['timestamp']) for d in data if d['user_id'] == i and d['application_id'] == a]
                ys = [d['time_used'] for d in data if d['user_id'] == i and d['application_id'] == a]
                for y in ys:
                    avg.append(y)

            print(a, np.mean(avg), np.std(avg))
            result.append({'timestamp': timestamp, 'application_id': a, 'mean': np.mean(avg), 'std': np.std(avg)})

    with open('data.json', 'w') as f:
        json.dump(result, f)

