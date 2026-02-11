import java.util.ArrayList;
import java.util.List;

public class ComplexJava<T> {
    private T value;

    public ComplexJava(T value) {
        this.value = value;
    }

    public T getValue() {
        return value;
    }

    public <U> U convert(U input) {
        return input;
    }

    public static <E> List<E> createList(E... elements) {
        List<E> list = new ArrayList<>();
        for (E element : elements) {
            list.add(element);
        }
        return list;
    }

    public void process(int x) {
        System.out.println("Int: " + x);
    }

    public void process(String s) {
        System.out.println("String: " + s);
    }

    public static void main(String[] args) {
        ComplexJava<Integer> intBox = new ComplexJava<>(42);
        Integer val = intBox.getValue();
        System.out.println(val);

        List<String> names = createList("Alice", "Bob", "Charlie");
        System.out.println(names);

        ComplexJava<String> strBox = new ComplexJava<>("Hello");
        strBox.process(10);
        strBox.process("World");
    }
}
