
Une fois connecté, installer rust : 
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
Lorsque demandé, tapper 1. 

<img width="661" height="95" alt="image" src="https://github.com/user-attachments/assets/88d35c55-a3f1-4c85-8634-f627d7fb84f2" />

Pour complier du code installer build-essential : 

```
sudo apt-get update
```
```
sudo apt-get install build-essential
```

Une fois installé, créer le repertoire prévu : 

```
mkdir projects
cd projects
```

Pour compiler : 

```
rustc <nom_du_programme>
```

# Appendix télécharger un repo/fichier depuis git vers la vm

```
wget "https://github.com/VincentGvr/Embedded/tree/main/Rust/Projects/HelloWorld/main.rs"
```

# Appendix pousser un fichier local via SSH 

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
