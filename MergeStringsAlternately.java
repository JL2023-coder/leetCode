import java.util.ArrayList;

public class MergeStringsAlternately {
    public String mergeAlternately(String word1, String word2) {
        StringBuilder sb = new StringBuilder();
        int l1 = word1.length();
        int l2 = word2.length();
        for(int i = 0; i < Math.max(l1, l2); i++){
            if(i < l1){
                sb.append(word1.charAt(i));
            }
            if(i < l2){
                sb.append(word2.charAt(i));
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        MergeStringsAlternately solution = new MergeStringsAlternately();
        String result = solution.mergeAlternately("abc", "pqr");
        System.out.println(result); // Expected output: "apbqcr"
    }

    public String gcdOfStrings(String str1, String str2) {
        ArrayList<String> list = new ArrayList();
        int lStr1 = str1.length();
        StringBuilder sub = new StringBuilder();
        for(int i = 0; i < lStr1; i++){
            if(lStr1 % sub.length() == 0){
                int recursive = lStr1 % sub.length();
                if(sub.repeat(sub, recursive).toString() == str1){
                    list.add(sub.toString());
                }
            }
            sub.append(str1.charAt(i));
        }
        int numInList = list.size();
        for(int n = numInList; n > 0; n--){
            String subStr = list.get(n);
            if(str2.contains(subStr)){
                return subStr;
            }
        }
        return "";
    }
}


