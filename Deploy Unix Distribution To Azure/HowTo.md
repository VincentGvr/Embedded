# Dans une souscription Azure : 

<!> Info : 
  - Si dans un laptop : x64 (plus puissant mais plus consommateur) 
  - Si dans un device : Arm (plus efficient, moins compatible, moins cher) 

Démarrer Powershell à la main ou bien depuis un cmd : 
```
powershell
```

Depuis Powershell, se connecter à Azure CLI puis définir la région et le nom du groupe de ressource : 
```
az login

$rgName = "rg-unix"
$azLocation = "westeurope"

az group create --name $rgName --location $azLocation

$vmName = "vm-debian-learnCode"
$imageName = "Debian:debian-13:12:latest"
$userName = "vgu"
$vmSize = "Standard_B1s" #"Standard_B1s" 

az vm create \
  --resource-group $rgName \
  --name $vmName \
  --image $imageName \
  --size Standard_B1s \
  --admin-username $userName \
  --generate-ssh-keys
```

## Appendix : Pour lister les distributions : 
```
$architecture = "x64" #"Arm64"
$sku = "13"
az vm image list --all --location $azLocation --publisher "Debian" --sku "13" --output "table" --query "max_by([], &version).{version:version} & [?architecture=='x64']"
```
