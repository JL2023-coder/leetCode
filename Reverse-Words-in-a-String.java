class Solution {
    public String reverseWords(String s) {
        StringBuilder newString = new StringBuilder();
        ArrayList<StringBuilder> words = getWordsFromString(s);
        for(int i = words.size() - 1; i >= 0; i--){
            newString.append(words.get(i) + \ \);
        }
        
        newString.setLength(newString.length() - 1);

        return newString.toString(); 
    }

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
}