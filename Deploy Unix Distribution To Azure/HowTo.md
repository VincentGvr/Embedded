# Dans une souscription Azure : 

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
```

