
trunk build --release
 
cd ../
rm -rf manger-deploy/*
rsync -a manger2/dist/ manger-deploy/
cd manger-deploy
git add .
git commit -m "update"
git push