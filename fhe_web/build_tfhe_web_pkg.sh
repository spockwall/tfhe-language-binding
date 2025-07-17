git clone https://github.com/zama-ai/tfhe-rs.git
cd tfhe-rs
make build_web_js_api   
cd ..
mv tfhe-rs/tfhe/pkg/tfhe* ./src
rm -rf tfhe-rs