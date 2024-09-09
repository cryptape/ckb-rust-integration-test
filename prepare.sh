git clone -b v0.117.2 https://github.com/gpBlockchain/ckb-rpc-mock-data.git
cd ckb-rpc-mock-data
pip install -r requirements.txt
pip install Werkzeug==2.2.2
python api/index.py > index.log 2>&1 &
cd ../
git clone https://github.com/gpBlockchain/ckb-light-client-rpc-mock-data.git
cd ckb-light-client-rpc-mock-data
git checkout 03526cf054f7e56b88b073ddfe4be9bdc7c8e8c0
pip install -r requirements.txt
python api/index.py > index.log 2>&1 &