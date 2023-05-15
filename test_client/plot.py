import matplotlib.pyplot as plt 
from pymongo import MongoClient
import datetime
import numpy as np
import time

if __name__ == '__main__':
    client = MongoClient()
    db = client['mn_system']
    collection = db['NetworkLog']

    while 1:
        data = list(collection.find({'timestamp': {'$gt': time.time() - 15}}))
        avg = []
        for a in range(10):
            for i in range(32):
                xs = [datetime.datetime.fromtimestamp(d['timestamp']) for d in data if d['user_id'] == i and d['application_id'] == a]
                ys = [d['time_used'] for d in data if d['user_id'] == i and d['application_id'] == a]
                for y in ys:
                    avg.append(y)


            print(a, np.mean(avg), np.std(avg))
            # plt.scatter(xs, ys, label=f"user id: {i}")

        # plt.legend()
        # plt.show()
