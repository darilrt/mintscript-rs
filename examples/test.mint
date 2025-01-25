// Opcion A
class Foo {
    boo: int

    def new(value: int) {
        return Foo {
            boo: value
        };
    }
}

// Opcion B
struct Foo {
    boo: int
}

def Foo.new(value: int) -> Foo {
    return Foo {
        boo: value
    };
}
