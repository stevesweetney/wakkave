@0xcf80bc85243221ab;

struct Request {
    union {
        login :union {
            credentials :group {
                username @0 :Text;
                password @1 :Text;
            }
            token @2 :Text;
        }

        logout @3 :Text; # Session token
    }
}
