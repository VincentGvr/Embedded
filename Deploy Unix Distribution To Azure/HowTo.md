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
$imageName = "Debian:debian-13:13:latest"
$userName = "vgu"
$vmSize = "Standard_B1s" #"Standard_B1s" 

az vm create `
  --resource-group $rgName `
  --name $vmName `
  --image $imageName `
  --size $vmSize `
  --admin-username $userName `
  --generate-ssh-keys
```

Une fois la VM créée, récupérer son IP Publique et se connecter : 

```
$vmPubIp = az vm show `
  --resource-group $rgName `
  --name $vmName `
  --show-details `
  --query publicIps -o tsv

ssh $userName@$vmPubIp
```

Une fois connecté, installer rust : 
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
Lorsque demandé, tapper 1. 

<img width="661" height="95" alt="image" src="https://github.com/user-attachments/assets/88d35c55-a3f1-4c85-8634-f627d7fb84f2" />

Une fois installé, créer le repertoire prévu : 

```
mkdir projects
cd projects
```

Pour déployer un fichier local en SSH vers la VM, ouvrir un nouvel onglet dans l'invit de commande 

Se connecter à nouveau :

```
az login

$rgName = "rg-unix"
$vmName = "vm-debian-learnCode"
$userName = "vgu"

$vmPubIp = az vm show `
  --resource-group $rgName `
  --name $vmName `
  --show-details `
  --query publicIps -o tsv
```

une fois positionné dans le folder où le fichier a été créé, pousser le fichier à l'aide de la commande : 
```
scp -r ./ $userName@$($vmPubIp):~/projects/helloworld
```
ou rsync qui ne déploie que les changes : 
```
rsync -avz ./ $userName@$($vmPubIp):~/projects/helloworld
```
## Rust Appendix : to work offline : 

$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0

## Appendix : Pour lister les distributions : 
```
$architecture = "x64" #"Arm64"
$sku = "13"
az vm image list --all --location $azLocation --publisher "Debian" --sku "13" --output "table" --query "max_by([], &version).{version:version} & [?architecture=='x64']"
```
