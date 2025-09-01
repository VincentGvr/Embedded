$iotHubName = "mqttDemoIotHub"

az iot hub create `
  --name $iotHubName `
  --resource-group $rgName `
  --sku B1 `
  --location $azLocation

<img width="1305" height="635" alt="image" src="https://github.com/user-attachments/assets/8bdeb179-d518-4dcd-9dd1-3875611a63c6" />
<img width="1042" height="573" alt="image" src="https://github.com/user-attachments/assets/6594ba40-d83e-4684-a72b-670d6f4c4a4c" />
