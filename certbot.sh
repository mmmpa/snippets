wget https://dl.eff.org/certbot-auto
chmod a+x certbot-auto

sudo easy_install virtualenv
mkdir encry
virtualenv --no-site-packages encry
cd encry
source bin/activate
pip install certbot
sudo /home/ec2-user/encry/bin/certbot certonly
