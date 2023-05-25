import matplotlib.pyplot as plt
import json

if __name__ == '__main__':
    with open('application_locations.json', 'r') as f:
        application_data = json.load(f)

    with open('data.json', 'r') as f:
        data = json.load(f)

    xs = [d['timestamp'] for d in application_data]
    ys = [d['edc_id'] for d in application_data]
    zs = [d['app_id'] for d in application_data]

    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')
    ax.scatter(xs,ys,zs)
    ax.set_xlabel("Time (s)")
    ax.set_ylabel("Edge data center id")
    ax.set_zlabel("Application id")
    plt.show()
    fig.savefig("plot_location.png")
    plt.clf()

    fig, ax = plt.subplots(nrows=2, ncols=4)
    id = 0
    for row in ax:
        for col in row:
            xs = [d['timestamp'] for d in data if d['application_id'] == id]
            ys = [d['mean'] for d in data if d['application_id'] == id]
            yerrs = [d['std'] for d in data if d['application_id'] == id]
            col.errorbar(xs, ys, yerr=yerrs)
            col.set_xlabel("Time (s)")
            col.set_ylabel("Mean time used to access last 15s")
            col.set_title(f"Application id {id}")
            id +=1
    fig.savefig("plot_mean.png")
    plt.show()


    # for id in range(8):
    #     ax = fig.add_subplot()
    #     plt.show()


