from flask import Flask
app = Flask(__name__)

@app.route('/say_hello')
def say_hello():
    return 'Hello, World!'

def run():
    app.run(debug=True)