image_name="registry.digitalocean.com/manger/app"
docker build --rm -t $image_name .
docker push $image_name
docker rmi $image_name