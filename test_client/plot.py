import matplotlib.pyplot as plt
import json
from datetime import datetime

def convert_timeseries_to_smaller(xs: [float]) -> [float]:
    return [x - min(xs) for x in xs]

if __name__ == '__main__':
    with open('application_locations.json', 'r') as f:
        application_data = json.load(f)

    with open('data.json', 'r') as f:
        data = json.load(f)

    xs = [d['timestamp'] for d in application_data]
    xs = convert_timeseries_to_smaller(xs)
    ys = [d['edc_id'] for d in application_data]
    zs = [d['app_id'] for d in application_data]


    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')
    ax.scatter(xs,ys,zs)
    ax.set_xlabel("Time (s)")
    ax.set_ylabel("Edge data center id")
    ax.set_zlabel("Application id")
    ax.set_title("Application location over time")
    fig.savefig("plot_location.png", dpi=750)
    plt.clf()

    for id in range(8):
        xs = [d['timestamp'] for d in data if d['application_id'] == id]
        xs = convert_timeseries_to_smaller(xs)
        ys = [d['mean'] for d in data if d['application_id'] == id]
        yerrs = [d['std'] for d in data if d['application_id'] == id]
        plt.errorbar(xs, ys, yerr=yerrs)
        plt.xlabel("Time (s)")
        plt.ylabel("Mean time used to access last 15s")
        plt.title(f"Application id {id}")
        plt.savefig(f"plot_mean_app_id_{id}.png", dpi=750)
        plt.clf()



    # for id in range(8):
    #     ax = fig.add_subplot()
    #     plt.show()


