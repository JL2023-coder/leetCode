import java.util.ArrayList;

public class test {

    private ArrayList<StringBuilder> getWordsFromString(String s){
        ArrayList<StringBuilder> words = new ArrayList<>();    
        StringBuilder word = new StringBuilder();    
        for(int i = 0; i < s.length(); i++){
            if(s.charAt(i) == ' ' && word.length() == 0){
                continue;
            }
            if(s.charAt(i) != ' '){
                word.append(s.charAt(i));
            }
            if(s.charAt(i) == ' ' || i == s.length() - 1){
                words.add(word);
                word = new StringBuilder();
            }
        }

        return words;
    }

    public static void main(String[] args) {
        test t = new test();
        String s = "the sky is blue";
        System.out.println(t.getWordsFromString(s));
    }
}
