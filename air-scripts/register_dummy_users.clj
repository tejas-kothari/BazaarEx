(xor
    (seq
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "init_service") [] result)
            )
            (seq
                (call relay ("op" "identity") [])
                (call %init_peer_id% (returnService "run") [result])
            )
        )
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "register_user") [seller_id seller_name] result)
            )
            (seq
                (call relay ("op" "identity") [])
                (call %init_peer_id% (returnService "run") [result])
            )
        )
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "register_user") [buyer_id buyer_name] result)
            )
            (seq
                (call relay ("op" "identity") [])
                (call %init_peer_id% (returnService "run") [result])
            )
        )
        (seq
            (seq
                (call relay ("op" "identity") [])
                (call xpact_node (xpact_service "register_user") [deliver_id deliver_name] result)
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