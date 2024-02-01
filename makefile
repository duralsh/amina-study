CARGO = cargo
FOUNDRY = forge
CONTRACTS_DIR = amina-contracts
RELEASE_TARGET = target/release/amina-study

rust:
	$(CARGO) build

release: 
	$(CARGO) build --release

solidity:
	cd $(CONTRACTS_DIR) && $(FOUNDRY) build

test:
	cd $(CONTRACTS_DIR) && $(FOUNDRY) test

clean:
	$(CARGO) clean
	cd $(CONTRACTS_DIR) && $(FOUNDRY) clean
