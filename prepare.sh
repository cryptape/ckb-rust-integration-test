git clone -b v0.116.0 https://github.com/cryptape/ckb-rpc-mock-data.git
cd ckb-rpc-mock-data
pip install -r requirements.txt
pip install Werkzeug==2.2.2
python api/index.py > index.log 2>&1 &
cd ../
git clone https://github.com/cryptape/ckb-light-client-rpc-mock-data
cd ckb-light-client-rpc-mock-data
pip install -r requirements.txt
python api/index.py > index.log 2>&1 &