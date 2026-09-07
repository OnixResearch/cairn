let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{ locator = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", identity = "not-an-identity" } | contracts.ownership
