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
        registration :group {
            username @4 :Text;
            password @5 :Text;
        }
    }
}

struct Response {
    union {
        login :union {
            success :group {
                token @0 :Text;
                user @4 :User;
            }
            error @1 :Text;
        }

        logout :union {
            success @2 :Void;
            error @3 :Text;
        }
    }
}

struct User {
    id @0 :Int32;
    username @1 :Text;
    karma @2 :Int32;
    streak @3 :Int16;
}
