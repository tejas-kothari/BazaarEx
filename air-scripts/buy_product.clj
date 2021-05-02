(xor
    (seq
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "buy_item") [user_id item_id dropoff_location] result)
            )
            (seq
                (call relay ("op" "identity") [])
                (call %init_peer_id% (returnService "run") [result])
            )
        )
    )
    (seq
        (call relay ("op" "identity") [])
        (call %init_peer_id% (returnService "run") ["XOR FAILED" %last_error%])
    )   
)