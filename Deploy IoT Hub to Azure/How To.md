
# installer les deux packages nécessaires : 
```
winget install --exact --id Microsoft.AzureCLI  # az cli
az extension add --name azure-iot               # l'extension azure iot
```
# authentification
`az login`

# création du groupe de ressources
```
$rgName = "rg-iot"
$azLocation = "westeurope"

az group create --name $rgName --location $azLocation
```
# création de l'iot hub
```
$iotHubName = "mqttDemoIotHub"

az iot hub create `
  --name $iotHubName `
  --resource-group $rgName `
  --sku B1 `
  --location $azLocation
```
# creation d'un device dans l'iot hub
```
$deviceName = "vmIoTSimulator"

az iot hub device-identity create `
  --device-id $deviceName `
  --hub-name $iotHubName
```
# récupération de la connection string du device
```
az iot hub device-identity connection-string show `
  --device-id $deviceName `
  --hub-name $iotHubName `
  --output table
```

<img width="1305" height="635" alt="image" src="https://github.com/user-attachments/assets/8bdeb179-d518-4dcd-9dd1-3875611a63c6" />
<img width="1042" height="573" alt="image" src="https://github.com/user-attachments/assets/6594ba40-d83e-4684-a72b-670d6f4c4a4c" />
