
trunk build --release
 
cd ../
rm -rf manger_deploy/*
rsync -a manger2/dist/ manger_deploy/
cd manger_deploy
git add .
git commit -m "update"
git push