<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()>
Partial Class db_auth
    Inherits System.Windows.Forms.Form

    'Форма переопределяет dispose для очистки списка компонентов.
    <System.Diagnostics.DebuggerNonUserCode()>
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Является обязательной для конструктора форм Windows Forms
    Private components As System.ComponentModel.IContainer

    'Примечание: следующая процедура является обязательной для конструктора форм Windows Forms
    'Для ее изменения используйте конструктор форм Windows Form.  
    'Не изменяйте ее в редакторе исходного кода.
    <System.Diagnostics.DebuggerStepThrough()>
    Private Sub InitializeComponent()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.edit = New System.Windows.Forms.CheckBox()
        Me.close_btn = New System.Windows.Forms.Button()
        Me.login_btn = New System.Windows.Forms.Button()
        Me.man_db = New System.Windows.Forms.RadioButton()
        Me.auto_db = New System.Windows.Forms.RadioButton()
        Me.db_text = New System.Windows.Forms.TextBox()
        Me.Label6 = New System.Windows.Forms.Label()
        Me.pass_Text = New System.Windows.Forms.TextBox()
        Me.Label5 = New System.Windows.Forms.Label()
        Me.user_Text = New System.Windows.Forms.TextBox()
        Me.Label4 = New System.Windows.Forms.Label()
        Me.port_Text = New System.Windows.Forms.TextBox()
        Me.Label3 = New System.Windows.Forms.Label()
        Me.host_Text = New System.Windows.Forms.TextBox()
        Me.Label2 = New System.Windows.Forms.Label()
        Me.Label1 = New System.Windows.Forms.Label()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(510, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 8
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(551, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 7
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(0, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 6
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(0, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(593, 35)
        Me.Stringer.TabIndex = 5
        Me.Stringer.TabStop = False
        '
        'edit
        '
        Me.edit.AutoSize = True
        Me.edit.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.edit.ForeColor = System.Drawing.Color.Navy
        Me.edit.Location = New System.Drawing.Point(17, 469)
        Me.edit.Name = "edit"
        Me.edit.Size = New System.Drawing.Size(256, 33)
        Me.edit.TabIndex = 6
        Me.edit.Text = "Редактировать данные"
        Me.edit.UseVisualStyleBackColor = True
        '
        'close_btn
        '
        Me.close_btn.Cursor = System.Windows.Forms.Cursors.Arrow
        Me.close_btn.FlatAppearance.BorderColor = System.Drawing.Color.Navy
        Me.close_btn.FlatAppearance.MouseDownBackColor = System.Drawing.Color.Transparent
        Me.close_btn.FlatAppearance.MouseOverBackColor = System.Drawing.Color.Transparent
        Me.close_btn.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.close_btn.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.close_btn.ForeColor = System.Drawing.Color.Navy
        Me.close_btn.Location = New System.Drawing.Point(333, 243)
        Me.close_btn.Name = "close_btn"
        Me.close_btn.Size = New System.Drawing.Size(253, 41)
        Me.close_btn.TabIndex = 10
        Me.close_btn.Text = "Выход из приложения"
        Me.close_btn.UseVisualStyleBackColor = True
        '
        'login_btn
        '
        Me.login_btn.Cursor = System.Windows.Forms.Cursors.Hand
        Me.login_btn.FlatAppearance.BorderColor = System.Drawing.Color.Navy
        Me.login_btn.FlatAppearance.MouseDownBackColor = System.Drawing.Color.Transparent
        Me.login_btn.FlatAppearance.MouseOverBackColor = System.Drawing.Color.Transparent
        Me.login_btn.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.login_btn.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.login_btn.ForeColor = System.Drawing.Color.Navy
        Me.login_btn.Location = New System.Drawing.Point(378, 196)
        Me.login_btn.Name = "login_btn"
        Me.login_btn.Size = New System.Drawing.Size(157, 41)
        Me.login_btn.TabIndex = 9
        Me.login_btn.Text = "Авторизация"
        Me.login_btn.UseVisualStyleBackColor = True
        '
        'man_db
        '
        Me.man_db.AutoSize = True
        Me.man_db.Enabled = False
        Me.man_db.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.man_db.ForeColor = System.Drawing.Color.Navy
        Me.man_db.Location = New System.Drawing.Point(340, 147)
        Me.man_db.Name = "man_db"
        Me.man_db.Size = New System.Drawing.Size(240, 33)
        Me.man_db.TabIndex = 8
        Me.man_db.Text = "Ручная настройка БД"
        Me.man_db.UseVisualStyleBackColor = True
        '
        'auto_db
        '
        Me.auto_db.AutoSize = True
        Me.auto_db.Checked = True
        Me.auto_db.Enabled = False
        Me.auto_db.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.auto_db.ForeColor = System.Drawing.Color.Navy
        Me.auto_db.Location = New System.Drawing.Point(340, 99)
        Me.auto_db.Name = "auto_db"
        Me.auto_db.Size = New System.Drawing.Size(214, 33)
        Me.auto_db.TabIndex = 7
        Me.auto_db.TabStop = True
        Me.auto_db.Text = "Авто-нстройка БД"
        Me.auto_db.UseVisualStyleBackColor = True
        '
        'db_text
        '
        Me.db_text.Enabled = False
        Me.db_text.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.db_text.ForeColor = System.Drawing.Color.Navy
        Me.db_text.Location = New System.Drawing.Point(17, 419)
        Me.db_text.Name = "db_text"
        Me.db_text.Size = New System.Drawing.Size(299, 37)
        Me.db_text.TabIndex = 5
        '
        'Label6
        '
        Me.Label6.AutoSize = True
        Me.Label6.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label6.ForeColor = System.Drawing.Color.Navy
        Me.Label6.Location = New System.Drawing.Point(12, 387)
        Me.Label6.Name = "Label6"
        Me.Label6.Size = New System.Drawing.Size(194, 29)
        Me.Label6.TabIndex = 25
        Me.Label6.Text = "Исп. база данных:"
        '
        'pass_Text
        '
        Me.pass_Text.Enabled = False
        Me.pass_Text.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.pass_Text.ForeColor = System.Drawing.Color.Navy
        Me.pass_Text.Location = New System.Drawing.Point(17, 347)
        Me.pass_Text.Name = "pass_Text"
        Me.pass_Text.Size = New System.Drawing.Size(299, 37)
        Me.pass_Text.TabIndex = 4
        '
        'Label5
        '
        Me.Label5.AutoSize = True
        Me.Label5.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label5.ForeColor = System.Drawing.Color.Navy
        Me.Label5.Location = New System.Drawing.Point(12, 315)
        Me.Label5.Name = "Label5"
        Me.Label5.Size = New System.Drawing.Size(96, 29)
        Me.Label5.TabIndex = 23
        Me.Label5.Text = "Пароль:"
        '
        'user_Text
        '
        Me.user_Text.Enabled = False
        Me.user_Text.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.user_Text.ForeColor = System.Drawing.Color.Navy
        Me.user_Text.Location = New System.Drawing.Point(17, 275)
        Me.user_Text.Name = "user_Text"
        Me.user_Text.Size = New System.Drawing.Size(299, 37)
        Me.user_Text.TabIndex = 3
        '
        'Label4
        '
        Me.Label4.AutoSize = True
        Me.Label4.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label4.ForeColor = System.Drawing.Color.Navy
        Me.Label4.Location = New System.Drawing.Point(12, 243)
        Me.Label4.Name = "Label4"
        Me.Label4.Size = New System.Drawing.Size(162, 29)
        Me.Label4.TabIndex = 21
        Me.Label4.Text = "Пользователь:"
        '
        'port_Text
        '
        Me.port_Text.Enabled = False
        Me.port_Text.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.port_Text.ForeColor = System.Drawing.Color.Navy
        Me.port_Text.Location = New System.Drawing.Point(17, 203)
        Me.port_Text.Name = "port_Text"
        Me.port_Text.Size = New System.Drawing.Size(299, 37)
        Me.port_Text.TabIndex = 2
        '
        'Label3
        '
        Me.Label3.AutoSize = True
        Me.Label3.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label3.ForeColor = System.Drawing.Color.Navy
        Me.Label3.Location = New System.Drawing.Point(12, 171)
        Me.Label3.Name = "Label3"
        Me.Label3.Size = New System.Drawing.Size(71, 29)
        Me.Label3.TabIndex = 19
        Me.Label3.Text = "Порт:"
        '
        'host_Text
        '
        Me.host_Text.Enabled = False
        Me.host_Text.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.host_Text.ForeColor = System.Drawing.Color.Navy
        Me.host_Text.Location = New System.Drawing.Point(17, 131)
        Me.host_Text.Name = "host_Text"
        Me.host_Text.Size = New System.Drawing.Size(299, 37)
        Me.host_Text.TabIndex = 1
        '
        'Label2
        '
        Me.Label2.AutoSize = True
        Me.Label2.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label2.ForeColor = System.Drawing.Color.Navy
        Me.Label2.Location = New System.Drawing.Point(12, 99)
        Me.Label2.Name = "Label2"
        Me.Label2.Size = New System.Drawing.Size(146, 29)
        Me.Label2.TabIndex = 17
        Me.Label2.Text = "Хост/Сервер:"
        '
        'Label1
        '
        Me.Label1.AutoSize = True
        Me.Label1.Font = New System.Drawing.Font("Comic Sans MS", 15.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Label1.ForeColor = System.Drawing.Color.Navy
        Me.Label1.Location = New System.Drawing.Point(12, 38)
        Me.Label1.Name = "Label1"
        Me.Label1.Size = New System.Drawing.Size(306, 29)
        Me.Label1.TabIndex = 16
        Me.Label1.Text = "Введите данные от вашей БД"
        '
        'db_auth
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.SeaShell
        Me.ClientSize = New System.Drawing.Size(592, 514)
        Me.Controls.Add(Me.edit)
        Me.Controls.Add(Me.close_btn)
        Me.Controls.Add(Me.login_btn)
        Me.Controls.Add(Me.man_db)
        Me.Controls.Add(Me.auto_db)
        Me.Controls.Add(Me.db_text)
        Me.Controls.Add(Me.Label6)
        Me.Controls.Add(Me.pass_Text)
        Me.Controls.Add(Me.Label5)
        Me.Controls.Add(Me.user_Text)
        Me.Controls.Add(Me.Label4)
        Me.Controls.Add(Me.port_Text)
        Me.Controls.Add(Me.Label3)
        Me.Controls.Add(Me.host_Text)
        Me.Controls.Add(Me.Label2)
        Me.Controls.Add(Me.Label1)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.MaximizeBox = False
        Me.Name = "db_auth"
        Me.Text = "РУКИ ОТОРВУ"
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents trei As PictureBox
    Friend WithEvents EndIt As PictureBox
    Friend WithEvents ChatICO As PictureBox
    Friend WithEvents Stringer As PictureBox
    Friend WithEvents edit As CheckBox
    Friend WithEvents close_btn As Button
    Friend WithEvents login_btn As Button
    Friend WithEvents man_db As RadioButton
    Friend WithEvents auto_db As RadioButton
    Friend WithEvents db_text As TextBox
    Friend WithEvents Label6 As Label
    Friend WithEvents pass_Text As TextBox
    Friend WithEvents Label5 As Label
    Friend WithEvents user_Text As TextBox
    Friend WithEvents Label4 As Label
    Friend WithEvents port_Text As TextBox
    Friend WithEvents Label3 As Label
    Friend WithEvents host_Text As TextBox
    Friend WithEvents Label2 As Label
    Friend WithEvents Label1 As Label
End Class
