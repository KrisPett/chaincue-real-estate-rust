kubectl delete all --all

eval $(minikube docker-env)
docker build -t backend-api-rust .
minikube image ls
minikube addons enable ingress
sudo nano /etc/hosts

```
192.168.00.0 local.chaincue-rust.com
```

kubectl -f postgres.yml apply
kubectl exec -it postgres-backend-0 -- psql -U admin -d postgres -c "CREATE DATABASE \"chaincue-real-estate-rust-postgres\";"

kubectl -f backend.yml apply


minikube image rm docker.io/library/backend-api-rust:latest
