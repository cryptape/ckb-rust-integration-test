git clone -b v0.111.0 https://github.com/gpBlockchain/ckb-rpc-mock-data.git
cd ckb-rpc-mock-data
pip install -r requirements.txt
python api/index.py > index.log 2>&1 &