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

struct Response {
    union {
        login :union {
            token @0 :Text;
            error @1 :Text;
        }

        logout :union {
            success @2 :Void;
            error @3 :Text;
        }
    }
}
