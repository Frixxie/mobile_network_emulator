from flask import Flask 
from flask import request

app = Flask(__name__)

@app.route('/pdn_connection_event', methods=['POST'])
def pdn_connection_event():
    print(request.headers)
    print(request.get_json())
    return 'OK'

@app.route('/location_reporting', methods=['POST'])
def locaiton_information():
    print(request.headers)
    print(request.get_json())
    return 'OK'

app.run(host='0.0.0.0', port=8567, debug=True)
