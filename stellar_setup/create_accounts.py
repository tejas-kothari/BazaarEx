from stellar_sdk.keypair import Keypair
import requests
from stellar_sdk.server import Server
import json

server = Server("https://horizon-testnet.stellar.org")
accounts = {}

for i in range(3):
    # Create a Keypair
    pair = Keypair.random()
    print(f"Secret: {pair.secret}")
    print(f"Public Key: {pair.public_key}")

    # Create Account
    public_key = pair.public_key
    response = requests.get(f"https://friendbot.stellar.org?addr={public_key}")
    if response.status_code == 200:
        print(f"SUCCESS! You have a new account :)\n{response.text}")

        # Get Account details
        account = server.accounts().account_id(public_key).call()
        for balance in account['balances']:
            print(f"Type: {balance['asset_type']}, Balance: {balance['balance']}")

        # Save Account Keys
        accounts[i] = {"sk": pair.secret, "pk": pair.public_key}
        
    else:
        exit(f"ERROR! Response: \n{response.text}")

# Write Account Keys to json
with open('accounts.json', 'w') as outfile:
    json.dump(accounts, outfile)