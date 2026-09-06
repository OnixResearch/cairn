let C = import "../../contracts/check-evidence-producer.ncl" in
{ id = "short", deadline_ms = 0, teardown_ms = 100, stdout_bytes = 4096, stderr_bytes = 4096 } | C.bounds
