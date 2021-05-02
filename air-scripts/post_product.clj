(xor
    (seq
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "post_item") [user_id pickup_location price description] result)
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