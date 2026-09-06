let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{ max_artifacts = 65, max_artifact_bytes = 1024, max_total_bytes = 4096 } | contracts.bounds
