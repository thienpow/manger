
trunk build --release
 
cd ../
cd manger-deploy
git pull
cd ../
rm -rf manger-deploy/*
rsync -a manger2/dist/ manger-deploy/
cd manger-deploy
git add .
git commit -m "update"
git push