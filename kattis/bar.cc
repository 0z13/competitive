#include <iostream>


class Event
{
public:
	std::string getType();
	int getTime();
	//virtual std::string getDescription()=0;

private:
	std::string m_type;
	int m_time;
};


int main (void) {
    Event e;
    std::cout << "hello" << std::endl;
    return 0;
}
