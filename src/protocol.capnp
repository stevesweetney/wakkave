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

        fetchPosts @6 :Text; # Session token
        createPost :group {
            token @7 :Text;
            content @8 :Text;
        }
        userVote :group {
            token @9 :Text;
            vote @10 :Vote;
            postId @11 :Int32;
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

        fetchPosts :union {
            success :group {
                token @5 :Text;
                posts @6 :List(Post);
            }
            error @7 :Text;
        }

        createPost :union {
            success :group {
                token @8 :Text;
                post @9 :Post;
            }
            error @10 :Text;
        }

        userVote :union {
            success @11 :Text; # Session Token
            error @12 :Text;
        }

        update @13 :Update;
    }
}

struct User {
    id @0 :Int32;
    username @1 :Text;
    karma @2 :Int32;
    streak @3 :Int16;
}

enum Vote {
    up @0;
    none @1;
    down @2;
}

struct Post {
    id @0 :Int32;
    content @1 :Text;
    valid @2 :Bool;
    vote @3 :Vote;
    userId @4 :Int32;
}

struct Update {
    union {
        invalid @0 :List(Int32);
        users @1 :List(User);
        newPost @2 :Post;
    }
}
