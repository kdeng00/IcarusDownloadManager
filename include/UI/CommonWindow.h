#ifndef COMMONWINDOW_H_
#define COMMONWINDOW_H_

#include<memory>

#include<QDialog>
#include<QHBoxLayout>
#include<QVBoxLayout>
#include<QComboBox>
#include<QPushButton>


class CommonWindow
{
public:
    CommonWindow() = default;
    ~CommonWindow() = default;
protected:
    virtual void connections()=0;
    std::unique_ptr<QComboBox> selectionBoxQt;
    std::unique_ptr<QPushButton> actionButtonQt;
    std::unique_ptr<QVBoxLayout> mainLayoutQt;
    std::unique_ptr<QVBoxLayout> subLayoutOneQt;
    std::unique_ptr<QVBoxLayout> subLayoutTwoQt;
    int windowHeight, windowWidth;
};
#endif
