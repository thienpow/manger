
trunk build --release
 
cd ../
cd manger-deploy
git pull
cd ../
rm -rf manger-deploy/*
cp -R manger/dist/* manger-deploy/
cd manger-deploy
git add .
git commit -m "update"
git push