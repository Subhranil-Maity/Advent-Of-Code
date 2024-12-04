#include<bits/stdc++.h>
using namespace std;
const string sample = "..\\samples\\day1.txt";

vector<string> splitString(string &s, char d){
    vector<string> tokens;
    stringstream ss(s);
    string token;
    while(getline(ss, token, d)){
        tokens.push_back(token);
    }
    return tokens;
}

// Function that appends a string to another using a reference
void appendString(std::string& base, const std::string toAppend) {
    base += toAppend;
}


int read(string &value, string path){
    ifstream file(sample);  // Open the file
    if (!file.is_open()) {
        cerr << "Unable to open file.\n";
        return 1;  // Return an error code
    }

    string line;
    
    while (getline(file, line)) {  // Read line by line
        // cout << line << endl;
        line +="\n";
        appendString(value, line);

    }

    file.close();  // Close the file
    return 0;
}


int main() {
    string sample1 = "";
    read(sample1, sample);
    vector<string> lines = splitString(sample1, '\n');
    
   vector<int> left_list;
   vector<int> right_list;
    for(auto it = lines.begin(); it != lines.end(); ++it){
        // cout <<*it << endl;
        auto something = splitString(*it, ' ');
        for(auto it2 = something.begin(); it2 != something.end(); ++it2){
            cout << "abcd "<<*it2 << endl;
        }

        // left_list.push_back(stoi(something.at(0)));
        // right_list.push_back(stoi(something.at(1)));
    }

   

   
    return 0;
}