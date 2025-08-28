# Dans une souscription Azure : 

Depuis un cmd : 

```
powershell
```

```
az login
$rgName = "rg-unix"
$azLocation = "westeurope"
az group create --name $rgName --location $azLocation
```

